FROM rust:latest

#Expose http and https ports
EXPOSE 8080
EXPOSE 443

#Install cargo and other libaries
RUN apt update
RUN apt upgrade -y
RUN apt install gcc -y
RUN apt install pkg-config -y
RUN apt install libssl-dev -y
RUN apt install git -y

#RUN rustup target add aarch64-unknown-linux-gnu

#Download project
RUN git clone https://github.com/largenumberhere/facts-bot.git
WORKDIR facts-bot

#Make sure there is no artifacts left over from other builds because they can break things if using a different version of clibs
RUN cargo clean

#Build the project
RUN cargo build --release
#RUN cargo build --release --target=aarch64-unknown-linux-gnu

#Add secrets
ADD keys/* ./

#Run on machiene startup
CMD cargo run --release