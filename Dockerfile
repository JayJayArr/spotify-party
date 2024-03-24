# Base image
FROM node:alpine

# Create app directory
WORKDIR /usr/src/app

# A wildcard is used to ensure both package.json AND package-lock.json are copied
COPY --chown=node:node ./spotify-party-bcknd/package*.json ./
# copy frontend
COPY --chown=node:node ./spotify-party/dist/spotify-party /usr/src/spotify-party/dist/spotify-party


# Install app dependencies
RUN npm install

# Bundle app source
COPY --chown=node:node ./spotify-party-bcknd/ .

WORKDIR /usr/src/app/spotify-party-bcknd
# Creates a "dist" folder with the production build
RUN npm run build


# Create app directory
WORKDIR /usr/src/app

USER node

EXPOSE 3000
# Start the server using the production build
CMD [ "node", "/usr/src/app/dist/main.js" ]