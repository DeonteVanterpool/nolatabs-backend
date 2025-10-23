provider "aws" {
    region = "us-east-1"
}

resource "aws_vpc" "main" {
 cidr_block           = var.vpc_cidr
 enable_dns_hostnames = true
 tags = {
   name = "main"
 }
}

resource "aws_subnet" "subnet" {
 vpc_id                  = aws_vpc.main.id
 cidr_block              = cidrsubnet(aws_vpc.main.cidr_block, 8, 1)
 map_public_ip_on_launch = true
 availability_zone       = "us-east-1"
}

resource "aws_subnet" "subnet2" {
 vpc_id                  = aws_vpc.main.id
 cidr_block              = cidrsubnet(aws_vpc.main.cidr_block, 8, 2)
 map_public_ip_on_launch = true
 availability_zone       = "us-east-1"
}


resource "aws_internet_gateway" "internet_gateway" {
 vpc_id = aws_vpc.main.id
 tags = {
   Name = "internet_gateway"
 }
}

resource "aws_route_table" "route_table" {
 vpc_id = aws_vpc.main.id
 route {
   cidr_block = "0.0.0.0/0"
   gateway_id = aws_internet_gateway.internet_gateway.id
 }
}

resource "aws_route_table_association" "subnet_route" {
 subnet_id      = aws_subnet.subnet.id
 route_table_id = aws_route_table.route_table.id
}

resource "aws_route_table_association" "subnet2_route" {
 subnet_id      = aws_subnet.subnet2.id
 route_table_id = aws_route_table.route_table.id
}

resource "aws_security_group" "security_group" {
 name   = "ecs-security-group"
 vpc_id = aws_vpc.main.id

 ingress {
   from_port   = 0
   to_port     = 0
   protocol    = -1
   self        = "false"
   cidr_blocks = ["0.0.0.0/0"]
   description = "any"
 }

 egress {
   from_port   = 0
   to_port     = 0
   protocol    = "-1"
   cidr_blocks = ["0.0.0.0/0"]
 }

 ingress {
    from_port   = 3892
    to_port     = 3892
    protocol    = -1
    self = "false"
    cidr_blocks = ["0.0.0.0/3892"]
 }
}

resource "aws_launch_template" "ecs_lt" {
 name_prefix   = "ecs-template"
 image_id      = "ami-062c116e449466e7f"
 instance_type = "t3.micro"

 key_name               = "ec2ecsglog"
 vpc_security_group_ids = [aws_security_group.security_group.id]
 iam_instance_profile {
   name = "ecsInstanceRole"
 }

 block_device_mappings {
   device_name = "/dev/xvda"
   ebs {
     volume_size = 30
     volume_type = "gp2"
   }
 }

 tag_specifications {
   resource_type = "instance"
   tags = {
     Name = "ecs-instance"
   }
 }

 user_data = filebase64("${path.module}/ecs.sh")
}
