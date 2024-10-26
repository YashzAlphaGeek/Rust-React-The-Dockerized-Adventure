# Use Node.js image as the base image
FROM node:16

# Set the working directory
WORKDIR /usr/src/frontend

# Copy package.json and install dependencies
COPY package.json ./
RUN npm install

# Copy the source code
COPY . .

# Build the frontend
RUN npm run build

# Serve the built frontend
CMD ["npm", "start"]
