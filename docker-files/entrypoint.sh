#!/bin/bash -e
telegraf --config /etc/telegraf/telegraf.conf &
exec "${@}"
