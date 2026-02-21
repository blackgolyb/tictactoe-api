# Tic Tac Toe Frontend

This is the frontend for the Tic Tac Toe API, built with Vite and configured to output a single HTML file.

## Project Structure

```
frontend/
├── index.html      # Main HTML structure
├── style.css       # All CSS styles
├── main.js         # Application logic
├── package.json    # Dependencies
├── vite.config.js  # Vite configuration
└── README.md       # This file
```

## Setup

1. Install dependencies:
```bash
npm install
```

## Development

Run the development server:
```bash
npm run dev
```

This will start a local development server at `http://localhost:5173` (or another port if 5173 is in use).

## Build

Build the project to a single HTML file:
```bash
npm run build
```

The output will be in the `dist/` directory as a single `index.html` file with all CSS and JavaScript inlined.

## Preview

Preview the production build:
```bash
npm run preview
```

## Features

- **Single File Output**: The build process creates a single HTML file with all CSS and JavaScript inlined
- **Code Splitting**: During development, files are separated for easier editing
- **Modern JavaScript**: Uses ES6+ features
- **Vite**: Fast hot module replacement (HMR) during development

## Configuration

The project uses `vite-plugin-singlefile` to bundle everything into a single HTML file. This is configured in `vite.config.js`.

## Notes

- The default API URL is set to `http://localhost:8128` in `main.js`
- To change the API URL, modify the `BASE_URL` variable in `main.js`
