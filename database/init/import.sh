#!/bin/sh
mongoimport \
	--db demo \
	--collection resource \
	--file "$(dirname "${BASH_SOURCE[0]}")"/data.json \
	--jsonArray
