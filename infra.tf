terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws",
      version = "~> 6.0"
    }
  }
}

provider "aws" {
  region = "eu-central-1"
}

resource "aws_s3_bucket" "john-burger-test-bucket" {
  bucket = "john-burger-test-bucket"
}

resource "aws_secretsmanager_secret" "john-burger-secrets" {
  name = "john-burger-secrets"
}

resource "aws_elasticache_cluster" "john-burger-cache" {
  name = "john-burger-cache"
}
