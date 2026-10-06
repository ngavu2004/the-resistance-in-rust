#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 || ! "$1" =~ ^[a-zA-Z0-9_]+$ ]]; then
    echo "Usage: $0 <player_name>"
    exit 1
fi

username="resistance_game_$1"

if id "$username" &>/dev/null; then
    echo "User already exists: $username"
    exit 0
fi



echo "Creating user: $username"
sudo useradd --create-home --shell /usr/sbin/nologin "$username"
echo "Created user: $username"