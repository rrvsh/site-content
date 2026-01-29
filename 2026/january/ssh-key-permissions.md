---
title: .ssh directory and private key permissions
slug: ssh-dir-perms
date: 2026-01-29
tags: [til]
---
The `~/.ssh` folder needs `700` and `~/.ssh/id_*` (the private key) needs `600`.

You can accomplish this with:

```bash
mkdir -p ~/.ssh
cp $SSH_KEY_LOCATION ~/.ssh/
chmod 700 ~/.ssh && chmod 600 ~/.ssh/id_*
ls -la ~/.ssh # to verify
```

Thank you to https://superuser.com/a/215506 who I have literally referenced tens of times for this and keep forgetting.
