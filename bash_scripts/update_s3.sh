#!/bin/bash
source .env
# This script updates .env files in an S3 bucket by replacing a specific key-value pair.
aws s3 cp .env $S3_BUCKET_ARN
