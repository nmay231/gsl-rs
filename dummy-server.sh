trap handle_interupt INT

handle_interupt() {
    echo Caught Ctrl-C. Cleaning up.
    sleep 2
    exit 0
}

echo game server starting
sleep 5
echo game server exiting
