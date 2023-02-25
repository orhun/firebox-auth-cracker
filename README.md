## `firebox-auth-cracker` 🔥

##### A CLI tool to brute force the authentication signature of [WatchGuard](https://www.watchguard.com/)'s Firebox

_Note: This is a quick and unfinished attempt for cracking the authentication signature of Firebox to bypass the firewall rules._

> Firebox is a powerful network security device that controls all traffic between the external network and the trusted network. If computers with mixed trust connect to your network, you can also configure an optional network interface that is separate from the trusted network.
> See https://www.watchguard.com/wgrd-products/firewall-appliances

### Attack Vector

[Authentication process](https://www.watchguard.com/help/docs/help-center/en-US/Content/en-US/Fireware/authentication/hotspot_external_web_server_config_c.html) of Firebox is shown in the following chart:

![diagram-hotspot-external-auth-interaction-flow](https://user-images.githubusercontent.com/24392180/221375134-4682ddcf-8ce4-4213-be6d-cfd6d9d34b6f.jpg)

1. A hotspot user tries to browse to a web page.
2. If this is a new hotspot user, the Firebox redirects the client browser to the Authentication URL on the external web server.
   This URL includes a query string that contains the access request.
3. The browser sends the access request to the external web server.
4. The external web server sends the Authentication page to the browser.
5. The hotspot user types the requested authentication information and submits the form to the external web server.
6. The external web server processes the authentication information and sends an HTML page that contains the decision URL to the browser.
7. The browser sends the access decision to the Firebox.
   The access decision URL contains the access decision, a checksum, and a redirect URL.
8. The Firebox reads the access decision, verifies the checksum, and sends the redirect URL to the client browser.
   Based on the outcome of the external authentication process, the redirect URL can be:
   - The original URL the user browsed to, if the external web server sent the original redirect URL.
   - A different redirect URL, if the external web server sent a different redirect URL.
   - The authentication failure URL, if authentication failed or access was denied.
9. The external web server sends a logoff URL to the Firebox to end the user hotspot session.

On step 2, we receive the following URL for the authentication page redirect:

```
http://10.0.2.80:8080/auth.html?xtm=http://10.0.3.1:4106/wgcgi.cgi&action=hotspot_auth&ts=1344238620&sn=70AB02716F745&mac=9C:4E:36:30:2D:26&redirect=http://www.google.com/
```

This access request URL includes these parameters:

- `xtm`: The URL on the Firebox where the external web server must send the access decision.
- `action`: The action type. The value is always `hotspot_auth`.
- `ts`: The time stamp for the request.
- `sn`: The serial number of the Firebox.
- `mac`: The MAC address of the client.
- `redirect`: The original URL the hotspot user tried to browse to.

Then later on step 7 (after the authentication is completed), we have receive the following result page URL:

```
http://10.0.3.1:4106/wgcgi.cgi?action=hotspot_auth&ts=1344238620&success=1&sess_timeout=1200&idle_timeout=600&&sig=a05d352951986e5fbf939920b260a6be3a9fffd3&redirect=http://www.google.com/
```

This URL includes the following parameters:

- `action`: The action type. The value must be `hotspot_auth`.
- `success`: The decision about hotspot access. Set the value to 1 to allow the user to get access the hotspot, or 0 to not allow access.
- `sess_timeout`: The session timeout value for the user hotspot connection. Specify the amount of time in seconds that a user can be connected to the hotspot for each session. Set the value to 1 to use the Session Timeout setting configured on the Firebox. Set the value to 0 to disable the session timeout value. When you set the value to 0, the user connection to the hotspot does not timeout.
- `idle_timeout`: The idle timeout value for the user hotspot connection. Specify the amount of time in seconds that a user session connection to the hotspot can be idle before the session is disconnected. Set the value to -1 to use the default Idle Timeout setting configured on the Firebox. Set the value to 0 to disable the idle timeout value. When you set the value to 0, the user connection to the hotspot does not expire when there is no traffic between the user client and the hotspot.
- `sig`: A hex encoded string in lower case. It is a SHA1 checksum based on the values of `ts`, `sn`, `mac`, `success`, `sess_timeout`, `idle_timeout`, and the **shared secret**. The shared secret you use to calculate the hash checksum must match the shared secret configured in the hotspot settings on the Firebox.
- `redirect`: The redirect URL you want the Firebox to send to the hotspot user after successful authentication. To redirect the browser to the original URL the user requested, use the value originally received in the access request URL. To redirect users to a different URL, specify that URL in this parameter.

---

**The idea is** we perform a brute force attack on the **signature** to find out the **shared secret** so that we can craft a custom signature and skip the authentication by directly sending a request to Firebox.

```
signature = SHA1(ts + sn + mac + success + sess-timeout + idle_timeout + shared_secret)
```

By this formula, we get:

```
a05d352951986e5fbf939920b260a6be3a9fffd3 = SHA1("1344238620" + "70AB02716F745" + "9C:4E:36:30:2D:26" + "1200" + "600" + shared_secret)
```

> The shared secret is the key the Firebox and the authentication server use to secure the authentication information that passes between them. The shared secret is case-sensitive and must be the same on the Firebox and the authentication server.

### Usage

```
Usage: firebox-auth-cracker [OPTIONS] --sig <SIG> --sn <SN> --ts <TS> --mac <MAC>
```

```
Options:
  -s, --sig <SIG>           Authentication signature
  -i, --input-file <INPUT>  Input file
      --sn <SN>             Serial number of the Firebox
  -t, --ts <TS>             Timestamp for the request
  -m, --mac <MAC>           MAC address of the client
  -h, --help                Print help
  -V, --version             Print version
```

### Examples

```sh
$ echo "test" | firebox-auth-cracker --sn "D0FE0CF6C42CB" --sig "d80cda05a95013000ecb0058fc46f9e89be8e641" --ts "1677336031" --mac "CA:FE:C0:FF:EE:00"

Checking d80cda05a95013000ecb0058fc46f9e89be8e641 (test)
Secret found!!! -> test
```

```sh
$ while true; do ( cat /dev/urandom | tr -dc 'a-zA-Z0-9' | head -c 8 ; echo "" ); done | firebox-auth-cracker --sn "D0FE0CF6C42CB" --sig "e3cb81859994095e79369bab7f72b4cb8260ed27" --ts "1677336031" --mac "CA:FE:C0:FF:EE:00"
```

### Roadmap

- [ ] Built-in key generator
- [ ] Multithreading

### License

Licensed under either of [Apache License Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or [The MIT License](http://opensource.org/licenses/MIT) at your option.

### Copyright

Copyright © 2023, [Orhun Parmaksız](mailto:orhunparmaksiz@gmail.com)
