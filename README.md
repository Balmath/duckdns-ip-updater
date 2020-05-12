# DuckDNS IP Updater 
Utility program that will update the ip address for Duck DNS

# Install on linux

You can run those commands to install the updater:

```
~ > git clone https://github.com/Balmath/duckdns-ip-updater.git
~ > cd duckdns-ip-updater
~/duckdns-ip-updater > make
~/duckdns-ip-updater > make install
```

# Edit the default config

You can change the default config with your domains to update and the token.
The file is located in the `$HOME\.config\duckdns-ip-updater\default.conf`.

# Add as cron task
This command will run the updater every 5 minutes.

```
> crontab -e
```

Add these lines to the crontab:

```
# Run DuckDNS IP updater every 5 minutes
*/5 * * * *     $HOME/bin/duckdns-ip-updater
```
