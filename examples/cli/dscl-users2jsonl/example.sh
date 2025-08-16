#!/bin/sh

./dscl-users2jsonl |
	jq -c |
	tail -3 |
	dasel --read=json --write=yaml |
	bat --language=yaml
