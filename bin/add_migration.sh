#!/usr/bin/env bash

set -x
set -eo pipefail

export DATABASE_URL="postgres://postgres:password@localhost:5440/newsletter"
echo DATABASE_URL

sqlx migrate add create_subsciptions_table
