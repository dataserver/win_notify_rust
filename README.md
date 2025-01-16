# Windows Notification Server

A simple Rust app to receive Windows notifications on your network using REST requests.

## How it works

The app creates an HTTP server on port 8081 and a system tray icon to control it.

POST requests sent to the host endpoint `xxx:8081/notification` will create a Windows notification on the machine.

## POST request format
```
POST http://your_network_ip:8081/notification
content-type: application/json

{
    "title": "Notification Title",
    "body_message": "Your message here."
}
```

Remember to copy the `/assets` directory so the app can use the appropriate icons.

- `assets/notificationlogo.png` -  Used for the Windows notification.
- `assets/trayicon.ico` - Used as the tray icon.

## Windows Startup

Add the app to Windows startup so the server is loaded automatically on start.
