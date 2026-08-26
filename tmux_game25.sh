#!/bin/zsh

SESSIONNAME="game25"
TARGET_DIR="/home/ash/programming_projects/rust_game/"
PROGRAM="hx"

echo "\x1b]2;tmux game_25\x07"

# Check if the session already exists
tmux has-session -t $SESSIONNAME &> /dev/null

if [ $? != 0 ]; then
    # Create a new detached session
    tmux new-session -s $SESSIONNAME -n window1 -d -c $TARGET_DIR

    # Split the window into two panes
    tmux split-window -h -c $TARGET_DIR

    # Run the program in the right pane
    tmux send-keys -t $SESSIONNAME:0.1 "cd $TARGET_DIR" C-m
    tmux send-keys -t $SESSIONNAME:0.1 "$PROGRAM" C-m

    # Select the left pane
    tmux select-pane -t $SESSIONNAME:0.0
fi

# Attach to the session
tmux attach -t $SESSIONNAME
