import { Box, AppBar, Toolbar, Typography, Container } from '@mui/material'

const Layout = () => {
  return (
    <Box sx={{ display: 'flex', flexDirection: 'column', minHeight: '100vh' }}>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6">
            Rust Fullstack App
          </Typography>
        </Toolbar>
      </AppBar>
      
      <Container component="main" sx={{ mt: 4, mb: 4, flex: 1 }}>
        <Typography variant="h4" component="h1" gutterBottom>
          Welcome to Rust Fullstack App
        </Typography>
        <Typography variant="body1">
          This is a fullstack application built with Rust backend and React frontend.
        </Typography>
      </Container>

      <Box component="footer" sx={{ py: 3, px: 2, mt: 'auto', backgroundColor: 'primary.main', color: 'white' }}>
        <Container maxWidth="sm">
          <Typography variant="body2" align="center">
            © {new Date().getFullYear()} Rust Fullstack App
          </Typography>
        </Container>
      </Box>
    </Box>
  )
}

export default Layout
