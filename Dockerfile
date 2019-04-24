FROM docker:18.09.4-dind


RUN apk update

RUN mkdir /root/dind_images/
COPY dind_images /root/dind_images/

WORKDIR /root/dind_images/
RUN chmod 777 start_dind.sh
