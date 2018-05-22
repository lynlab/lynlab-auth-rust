# API Specification
## List

  - GET /v1/activate/{token}
  - GET /v1/me
  - POST /v1/login
  - POST /v1/register


## Error Response Model

```json
{
  "error": "string"
}
```

  - `error` : Error code.


## GET /v1/activate/{token}

Activate the account.

### Request

  - `token`(string) : activation token.

### Response

  - **302 Found** : Account activated.
  - **400 BAD REQUEST** : The activation token is invalid.


## GET /v1/me

Get user information.

### Request

```json
Authorization: Bearer YOUR_ACCESS_TOKEN
```

### Response

  - **200 OK**
    ```json
    {
      "id": "string",
      "username": "string",
      "email": "string"
    }
    ```
  - **401 UNAUTHORIZED**


## POST /v1/login

Login user.

### Request

```json
{
  "username": "string",
  "password": "string"
}
```

### Response

  - **200 OK**
    ```json
    {
      "access_token": "string"
    }
    ```
  - **401 UNAUTHORIZED**
    - `AU0011` : No such username.
    - `AU0012` : Invalid password.



## POST /v1/register

Register new account.

### Request

```json
{
  "username": "string",
  "password": "string",
  "email": "string",
  "redirection_url": "string"
}
```

  - `redirection_url` : URL to redirect after email activation.

### Response
  - **200 OK** : The account added successfully and the confirmation email sent.
  - **400 BAD REQUEST** : Failed to register.
    - `AU0001` : Duplicated username or email.
