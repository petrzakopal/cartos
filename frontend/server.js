// server.js
import path from 'path';
import express from 'express';

const app = express();

// Serve static files from the build/client/assets folder
app.use('/assets', express.static(path.join(__dirname, 'build', 'client', 'assets')));

// Serve the rest of your static files (e.g., index.html)
app.use(express.static(path.join(__dirname, 'build')));

app.get('/', function (req, res) {
  res.sendFile(path.join(__dirname, 'build', 'client', 'index.html'));
});

app.listen(3000, () => {
  console.log('Server running on http://localhost:3000');
});
