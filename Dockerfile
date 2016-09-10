FROM jimmycuadra/rust
MAINTAINER Betsy Haibel <betsy.haibel@gmail.com>
RUN apt-get update && apt-get install -y ruby ruby-dev inkscape
RUN gem install bundler
COPY . constellation_bot
RUN cd constellation_bot && make production
