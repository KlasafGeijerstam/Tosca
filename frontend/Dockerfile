FROM node:15-alpine

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

COPY package.json package-lock.json /usr/src/app/
RUN npm install

COPY . /usr/src/app

ENTRYPOINT ["npm"]
CMD ["run", "start"]
