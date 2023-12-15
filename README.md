# Rust Geo API Webserver

Self-contained System for Geolocation API calls.

The Geo-Providers can be configured via web ui, including monthly payload limits.

## Status

![under under_construction](https://img.shields.io/badge/Status-under_construction-yellow)

## API Reference

### Landing Page

loads the landing page with some explanations

`/`

**Returns:**
HTML page

#### Get all providers

list all providers from database

```http
  GET /providers
```

| Parameter | Type  | Description                   |
| :-------- | :---- | :---------------------------- |
| `none`    | `-/-` | list all configured providers |

**Returns:**
JSON array

#### Get provider

```http
  GET /provider/${id}
```

| Parameter | Type     | Description                           |
| :-------- | :------- | :------------------------------------ |
| `id`      | `string` | **Required**. Id of provider to fetch |

**Returns:**
JSON

#### create provider

Create (add) a new provider to the database

```http
  POST /create/${JSON data}
```

| Parameter       | Type     | Description                                 |
| :-------------- | :------- | :------------------------------------------ |
| `name`          | `string` | **Required**. name of provider              |
| `description`   | `string` | **Required**. short description of provider |
| `api_key`       | `string` | **Required**. API key                       |
| `counter_limit` | `string` | **Required**. payload limit per month       |
| `counter`       | `string` | **Required**. initial payload (eg. 0)       |

## Authors

- [@Zheng-Bote](https://www.github.com/zheng-bote)

## License

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

## Environment Variables

To run this project, you will need to add the following environment variables to your .env file (see example: env_prod)

`DATABASE_URL="sqlite://data.db"`
