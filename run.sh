#!/bin/bash
if [ -f ~/.bash_profile ];
then
  . ~/.bash_profile
fi
step=30 #间隔秒数
for ((i = 0; i < 60; i = (i + step))); do
    cd /root/auto_buy
    /root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo run
    #cargo run
    sleep $step
done
exit 0