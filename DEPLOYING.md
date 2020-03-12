# Deploying

## Session Server

<!-- https://medium.com/@benmorel/creating-a-linux-service-with-systemd-611b5c8b91d6 -->

1. Set up an Ubuntu server that you may SSH into.

    Add the following to `~/.ssh/config`:

    ```
    Host will_server
      HostName 52.22.144.29
      User ubuntu
      IdentityFile ~/.ssh/your_private_key
    ```

2. Copy across the session sever binary and accompanying files.

    ```bash
    scp ./target/release/session_server will_server:/home/ubuntu
    scp ./app/session_server/session_server.service will_server:/home/ubuntu
    scp ./app/session_server/logger.yaml  will_server:/home/ubuntu
    ```

3. Log into the server, and set up the `session_server` service.

    ```bash
    # Log in
    ssh will_server
    ```

    Then

    ```bash
    sudo mv /home/ubuntu/session_server.service /etc/systemd/system/
    sudo systemctl start session_server
    sudo systemctl enable session_server
    ```
