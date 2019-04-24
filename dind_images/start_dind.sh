docker build -t loc_coder /root/dind_images/Coder/.
docker run -d  -p 8443:8443 -e CODER_PASSWORD="$CODER_PASSWORD" loc_coder:latest 