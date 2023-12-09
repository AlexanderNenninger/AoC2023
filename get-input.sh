#!/bin/bash
# exit when any command fails
set -e
daypadded=$(printf "%02d" $(($1)))
curl "https://adventofcode.com/2023/day/$1/input" -H "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:107.0) Gecko/20100101 Firefox/107.0" -H "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8" -H "Accept-Language: de,en-US;q=0.7,en;q=0.3" -H "Accept-Encoding: gzip, deflate, br" -H "Referer: https://adventofcode.com/2023/day/$1" -H "DNT: 1" -H "Connection: keep-alive" -H "Cookie: session=${COOKIE}" -H "Upgrade-Insecure-Requests: 1" -H "Sec-Fetch-Dest: document" -H "Sec-Fetch-Mode: navigate" -H "Sec-Fetch-Site: same-origin" -H "TE: trailers" | gunzip > "input/day${daypadded}.txt"