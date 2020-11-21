const {app, BrowserWindow} = require('electron')
function createWindow () {
  const mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
        // OPTIONAL: disable node integrations by commenting this line
        nodeIntegration: true
    }
  })
  // OPTIONAL: Hide the menubar
  // mainWindow.setMenu(null).
  mainWindow.loadFile('index.html')

  // OPTIONAL: Open the DevTools on startup
  // mainWindow.webContents.openDevTools()
}

app.whenReady().then(() => {
  createWindow()
  app.on('activate', function () {
    // On macOS it's common to re-create a window in the app when the
    // dock icon is clicked and there are no other windows open.
    if (BrowserWindow.getAllWindows().length === 0) createWindow()
  })
})

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on('window-all-closed', function () {
  if (process.platform !== 'darwin') app.quit()
})