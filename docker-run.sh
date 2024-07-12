#!/bin/sh

redis-server --daemonize yes
./yral-metadata-server
