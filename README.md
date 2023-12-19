<div id="top"></div>
<br />
<div align="center">

# Rust Geo API Webserver

Self-contained System for Geolocation API calls.

The Geo-Providers can be configured via web ui, including monthly payload limits.

</div>

## Status

![under under_construction](https://img.shields.io/badge/Status-under_construction-yellow)
**Still under heavy construction**

![deprecated](https://img.shields.io/badge/Info-limited_functionality-yellow)
**This application will be just rudimentary and mostly to learn Rust and the Rocket framework.**

## Features

- [x] Rust programming language with Rocket webframework

- [x] Diesel ORM and SQLx toolkit

- [x] Self-contained System with Web-based Userinterface

- [x] support for several Geolocation providers

- [x] each registered GEO provider with own configureable monthly payload limits

- [x] configuration based on SQLite3 database

- [x] template-based UI with Handlebars, JS and CSS support

- [ ] scaleable, clean architecture

- [ ] ReSTfull

- [x] Secured (XSS, ...)

- [ ] User authentication, rights and roles

- [ ] tbd.

## HISTORY:

> | Version | Date       | Developer | Comments |
> | ------- | ---------- | --------- | -------- |
> | 0.1.0   | 2023-12-16 | RZheng    | created  |

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

## Screenshots

**Startpage**

  <img src="https://raw.githubusercontent.com/Zheng-Bote/scs_api_geolocation/main/docs/index_hbs-index_html.png"  width="100%" height="auto" />

**create / add provider**

  <img src="https://raw.githubusercontent.com/Zheng-Bote/scs_api_geolocation/main/docs/add_provider.png"  width="100%" height="auto" />

**list all registered providers**

  <img src="https://raw.githubusercontent.com/Zheng-Bote/scs_api_geolocation/main/docs/listall_providers.png"  width="100%" height="auto" />

### the end

:vulcan_salute:

<p align="right">(<a href="#top">back to top</a>)</p>
