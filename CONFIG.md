# Configuration

Jotsy is configured using environment variables.
The below table shows the variables and the corresponding settings:

| Variable             | Description                                                                                    |
| -------------------- | ---------------------------------------------------------------------------------------------- |
| JOTSY_SKY_PORT       | Sets the Skytable database port                                                                |
| JOTSY_SKY_HOST       | Sets the Skytable database host                                                                |
| JOTSY_HOST           | Sets the host for the Jotsy app                                                                |
| JOTSY_PORT           | Sets the port for the Jotsy app                                                                |
| JOTSY_SIGNUP_ENABLED | Enables/disables registration for new users. Defaults to `true`                                |
| JOTSY_DEPLOY_PROD    | Sets the deploy mode. If set to `true`, all "production" settings are used. Defaults to `true` |

## Configuration and login loops

If you're running Jotsy on a server without HTTPS, for example by accessing it on a non-HTTPS enabled domain or directly from an "IP Address", you might run into a login loop because Jotsy will refuse to set secure authentication cookies on an insecure connection (for obvious reasons).

> Do note that this **doesn't apply to `localhost`**

To avoid this, consider setting `JOTSY_DEPLOY_PROD=false` to workaround this **only when you're testing it out**. If you're running it in production, **always set** `JOTSY_DEPLOY_PROD=true` to ensure secure transmission of cookies.
