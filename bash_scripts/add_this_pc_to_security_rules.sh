#!/bin/bash

# necessary variables: EC2_SGR_ID, EC2_GROUP_ID
source .env

MY_IP=$(curl -s https://checkip.amazonaws.com)
aws ec2 modify-security-group-rules \
    --group-id $EC2_GROUP_ID \
    --security-group-rules SecurityGroupRuleId=$EC2_SGR_ID,SecurityGroupRule="{Description=personal-computer,IpProtocol=tcp,FromPort=5432,ToPort=5432,CidrIpv4=$MY_IP/32}"
