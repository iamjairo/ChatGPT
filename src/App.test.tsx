import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/react';

const { mockGetCurrentWebview } = vi.hoisted(() => ({
  mockGetCurrentWebview: vi.fn(),
}));

vi.mock('@tauri-apps/api/webview', () => ({
  getCurrentWebview: mockGetCurrentWebview,
}));
vi.mock('~view/Titlebar', () => ({ default: () => <div data-testid="titlebar" /> }));
vi.mock('~view/Ask', () => ({ default: () => <div data-testid="ask" /> }));
vi.mock('~view/Settings', () => ({ default: () => <div data-testid="settings" /> }));

import App from './App';

describe('App routing', () => {
  it('renders Titlebar when webview label is "titlebar"', () => {
    mockGetCurrentWebview.mockReturnValue({ label: 'titlebar' });
    const { getByTestId } = render(<App />);
    expect(getByTestId('titlebar')).toBeInTheDocument();
  });

  it('renders Ask when webview label is "ask"', () => {
    mockGetCurrentWebview.mockReturnValue({ label: 'ask' });
    const { getByTestId } = render(<App />);
    expect(getByTestId('ask')).toBeInTheDocument();
  });

  it('renders Settings when webview label is "settings"', () => {
    mockGetCurrentWebview.mockReturnValue({ label: 'settings' });
    const { getByTestId } = render(<App />);
    expect(getByTestId('settings')).toBeInTheDocument();
  });

  it('renders null for unknown webview label', () => {
    mockGetCurrentWebview.mockReturnValue({ label: 'unknown' });
    const { container } = render(<App />);
    expect(container.firstChild).toBeNull();
  });
});
