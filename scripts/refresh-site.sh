#!/usr/bin/env bash

set -euo pipefail
umask 077

if (($# != 1)); then
	echo 'Usage: refresh-site.sh COMMIT_SHA' >&2
	exit 1
fi

commit_sha=$1
if [[ ! $commit_sha =~ ^[0-9a-f]{40}$ ]]; then
	echo 'The commit SHA must be 40 lowercase hexadecimal characters.' >&2
	exit 1
fi

if [[ -z ${SITE_REFRESH_PRIVATE_KEY:-} ]]; then
	echo 'SITE_REFRESH_PRIVATE_KEY is not set.' >&2
	exit 1
fi

work_dir=$(mktemp -d)
trap 'rm -rf "$work_dir"' EXIT
key_file=$work_dir/private-key.pem
body_file=$work_dir/body
signature_file=$work_dir/signature
response_file=$work_dir/response

printf '%s' "$SITE_REFRESH_PRIVATE_KEY" >"$key_file"
unset SITE_REFRESH_PRIVATE_KEY

if ! openssl pkey -in "$key_file" -check -noout >/dev/null 2>&1; then
	echo 'The site refresh private key is invalid.' >&2
	exit 1
fi
if ! openssl pkey -in "$key_file" -pubout 2>/dev/null |
	openssl pkey -pubin -text -noout 2>/dev/null |
	grep -q '^ED25519 Public-Key:'; then
	echo 'The site refresh private key is not an Ed25519 key.' >&2
	exit 1
fi

printf '%s' "$commit_sha" >"$body_file"
if ! openssl pkeyutl -sign -rawin \
	-inkey "$key_file" \
	-in "$body_file" \
	-out "$signature_file"; then
	echo 'Could not sign the site refresh request.' >&2
	exit 1
fi
signature=$(openssl base64 -A -in "$signature_file")

max_attempts=4
for ((attempt = 1; attempt <= max_attempts; attempt++)); do
	http_status=''
	if http_status=$(curl \
		--silent \
		--show-error \
		--connect-timeout 10 \
		--max-time 20 \
		--output "$response_file" \
		--write-out '%{http_code}' \
		--request POST \
		--header 'Content-Type: text/plain' \
		--header "X-Site-Content-Signature: $signature" \
		--data-binary "@$body_file" \
		https://rrv.sh/webhooks/site-content); then
		curl_status=0
	else
		curl_status=$?
	fi

	if [[ $http_status =~ ^3[0-9]{2}$ ]]; then
		echo "Site refresh endpoint returned a redirect ($http_status)." >&2
		exit 1
	fi
	if [[ $http_status =~ ^4[0-9]{2}$ && ! $http_status =~ ^(408|425|429)$ ]]; then
		echo "Site refresh endpoint rejected the request ($http_status)." >&2
		exit 1
	fi

	if ((curl_status != 0)); then
		case $curl_status in
		5 | 6 | 7 | 18 | 28 | 52 | 55 | 56) ;;
		*)
			echo "Site refresh request failed with curl error $curl_status." >&2
			exit 1
			;;
		esac
	elif [[ $http_status == 202 ]]; then
		exit 0
	elif [[ ! $http_status =~ ^(408|425|429|5[0-9]{2})$ ]]; then
		echo "Site refresh endpoint returned unexpected status $http_status." >&2
		exit 1
	fi

	if ((attempt < max_attempts)); then
		sleep $((attempt * 2))
	fi
done

echo '::warning::The site refresh endpoint remained temporarily unavailable after four attempts.'
exit 0
