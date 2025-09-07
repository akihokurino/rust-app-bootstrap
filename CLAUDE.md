# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based AWS serverless application template with the following architecture:

- **Workspace Structure**: Multi-crate Cargo workspace with 5 main crates:
  - `app`: Core application library with domain logic, infrastructure adapters, and shared types
  - `api`: GraphQL API server using Actix Web and async-graphql
  - `async_task`: Asynchronous task handler for SNS messages
  - `sync_task`: Synchronous task handler
  - `batch_task`: Batch processing task handler
  - `migration`: SeaORM database migrations

- **Infrastructure**: AWS serverless stack including Lambda functions, RDS PostgreSQL, S3, SNS, and SSM Parameter Store for configuration management

## Build and Development Commands

### Local Development
```bash
# Build the entire workspace
make build

# Run API server locally (requires SSM parameter setup)
make run-api

# Run local PostgreSQL database
make run-db

# Run database migrations
make run-migration

# Reset local database
make reset-db
```

### AWS Deployment
```bash
# Deploy to AWS (builds and deploys via SAM)
make deploy

# Connect to RDS via bastion host (for database administration)
make connect-rds
```

### Database Management
```bash
# Install SeaORM CLI
cargo install sea-orm-cli

# Generate new migration
sea-orm-cli migrate generate create_table_name

# Run migrations
cargo run -- refresh
```

## Architecture Details

### Dependency Injection Pattern
The `app` crate uses a resolver pattern with a singleton `Resolver` struct that contains all infrastructure adapters and repositories. Access via `app::resolver().await` which handles lazy initialization of AWS clients and database connections.

### AWS Integration
- **Configuration**: Environment variables loaded from SSM Parameter Store at startup
- **Database**: PostgreSQL via SeaORM with connection pooling
- **Storage**: S3 adapter for file operations  
- **Messaging**: SNS adapter for async task dispatch
- **Compute**: Lambda adapter for function invocations

### Database Layer
- **ORM**: SeaORM with PostgreSQL
- **Repositories**: Repository pattern for `user`, `order`, and `order_detail` entities
- **Migrations**: Managed via SeaORM migration crate in `migration/` directory

### API Layer
- **Framework**: Actix Web with GraphQL (async-graphql)
- **CORS**: Configured to allow any origin for development
- **Playground**: GraphQL playground available in development

## Important Development Notes

- The project uses Docker for Linux musl builds (required for Lambda deployment)
- SSM Parameter Store is used for sensitive configuration - set up `/app/server/dotenv` parameter before running locally
- AWS credentials and CloudFormation stacks must be properly configured for deployment
- The `USE_DOCKER=1` environment variable enables Docker-based builds