# SWPC Solarwind to Postgres

This project is a Proof of Concept (POC) pipeline for ingesting solar wind data from the SWPC (Space Weather Prediction Center) API into a PostgreSQL database.

## Overview

This Rust-based application fetches solar wind data from the SWPC API, processes it, and stores it in a PostgreSQL database. It's designed to run periodically to keep the database updated with the latest solar wind measurements.

## Features

- Fetches solar wind data from the SWPC API
- Processes and filters the data based on timestamp
- Stores the data in a PostgreSQL database
- Uses connection pooling for efficient database operations
- Parallel processing of data using Rayon

## Prerequisites

- Rust (latest stable version)
- PostgreSQL database
- Environment variable `SWPC_PGDB` set with the database connection string

## Project Structure

- `src/main.rs`: Contains the main application logic
- `src/swpc.rs`: Module for handling SWPC API data fetching and processing

## Setup

1. Clone the repository
2. Set up your PostgreSQL database
3. Set the `SWPC_PGDB` environment variable with your database connection string
4. Run `cargo build` to compile the project

## Usage

Run the application using:

`cargo run`

The application will:
1. Connect to the database
2. Fetch the latest timestamp from the existing data
3. Retrieve new solar wind data from the SWPC API
4. Filter and process the new data
5. Insert the new data into the database

## Database Schema

The application uses a table named `solarwind` with the following columns:

- `time_stamp`: UNIX timestamp
- `time_tag`: Formatted date-time string
- `speed`: Solar wind speed
- `density`: Solar wind density
- `temperature`: Solar wind temperature
- `bt`: Total interplanetary magnetic field
- `bz`: North-south component of the interplanetary magnetic field

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.