# Use the official Node.js image as the base image
FROM node:16 AS builder

# Set the working directory
WORKDIR /app

# Copy package.json and package-lock.json
COPY package.json package-lock.json ./

# Install the dependencies
RUN npm install

# Copy the rest of the application files
COPY . .

# Build the React app
RUN npm run build

# Use Nginx to serve the built app
FROM nginx:alpine

# Copy the build files from the previous stage
COPY --from=builder /app/build /usr/share/nginx/html

# Expose the port on which Nginx will run
EXPOSE 80

# Command to run Nginx
CMD ["nginx", "-g", "daemon off;"]
