import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { AdminGuard } from './admin-guard';
import { apiClient } from '@prospector/api-client';

// Mock del API Client para aislar el frontend
jest.mock('@prospector/api-client', () => ({
  apiClient: {
    get: jest.fn(),
  },
}));

describe('AdminGuard Component', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    sessionStorage.clear();
  });

  it('renders lock screen initially', () => {
    render(
      <AdminGuard>
        <div data-testid="protected-content">SECRET DATA</div>
      </AdminGuard>
    );
    // Ahora toBeInTheDocument será reconocido gracias a @testing-library/jest-dom
    expect(screen.getByText('Restricted Area')).toBeInTheDocument();
    expect(screen.queryByTestId('protected-content')).not.toBeInTheDocument();
  });

  it('unlocks content with correct password via API check', async () => {
    // Simulamos respuesta exitosa del backend (200 OK)
    (apiClient.get as jest.Mock).mockResolvedValue({ data: [] });

    render(
      <AdminGuard>
        <div data-testid="protected-content">SECRET DATA</div>
      </AdminGuard>
    );

    const input = screen.getByPlaceholderText('ENTER PASSPHRASE');
    // Usamos la contraseña por defecto del .env o el fallback del código
    fireEvent.change(input, { target: { value: 'Netflix69' } });

    const button = screen.getByText('AUTHENTICATE');
    fireEvent.click(button);

    await waitFor(() => {
      expect(screen.getByTestId('protected-content')).toBeInTheDocument();
    });
  });

  it('shows error on invalid password', async () => {
    render(<AdminGuard>SECRET</AdminGuard>);

    const input = screen.getByPlaceholderText('ENTER PASSPHRASE');
    fireEvent.change(input, { target: { value: 'WrongPass' } });

    const button = screen.getByText('AUTHENTICATE');
    fireEvent.click(button);

    await waitFor(() => {
        expect(screen.getByText(/ACCESS DENIED/i)).toBeInTheDocument();
    });
  });
});
