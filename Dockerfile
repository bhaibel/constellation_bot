FROM ruby:2.3.1
MAINTAINER Betsy Haibel <betsy.haibel@gmail.com>
RUN apt-get update && apt-get install -y inkscape
COPY . constellation_bot
RUN cd constellation_bot && bundle install --without test development
