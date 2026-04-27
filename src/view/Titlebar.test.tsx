import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, act } from '@testing-library/react';

const { mockInvoke, mockIsFullscreen, mockListen } = vi.hoisted(() => ({
  mockInvoke: vi.fn(),
  mockIsFullscreen: vi.fn().mockResolvedValue(false),
  mockListen: vi.fn().mockResolvedValue(vi.fn()),
}));

vi.mock('@tauri-apps/api/core', () => ({ invoke: mockInvoke }));
vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: vi.fn(() => ({
    isFullscreen: mockIsFullscreen,
    listen: mockListen,
    onThemeChanged: vi.fn().mockResolvedValue(vi.fn()),
    theme: vi.fn().mockResolvedValue('light'),
  })),
}));
vi.mock('@tauri-apps/plugin-shell', () => ({ open: vi.fn() }));
vi.mock('~hooks/useInfo', () => ({ default: () => ({ platform: 'linux', isMac: false }) }));

vi.mock('~icons/Reload', () => ({ default: (p: any) => <button onClick={p.onClick}>Reload</button> }));
vi.mock('~icons/Pin', () => ({ default: (p: any) => <button onClick={p.onClick}>Pin</button> }));
vi.mock('~icons/UnPin', () => ({ default: (p: any) => <button onClick={p.onClick}>UnPin</button> }));
vi.mock('~icons/Link', () => ({ default: () => <span>Link</span> }));
vi.mock('~icons/Ask', () => ({ default: (p: any) => <button onClick={p.onClick}>Ask</button> }));
vi.mock('~icons/Setting', () => ({ default: (p: any) => <button onClick={p.onClick}>Setting</button> }));
vi.mock('~icons/ThemeSystem', () => ({ default: (p: any) => <button onClick={p.onClick}>ThemeSystem</button> }));
vi.mock('~icons/ThemeLight', () => ({ default: (p: any) => <button onClick={p.onClick}>ThemeLight</button> }));
vi.mock('~icons/ThemeDark', () => ({ default: (p: any) => <button onClick={p.onClick}>ThemeDark</button> }));
vi.mock('~icons/ArrowLeft', () => ({ default: (p: any) => <button onClick={p.onClick} className={p.className}>ArrowLeft</button> }));

const baseConf = {
  theme: 'system',
  stay_on_top: false,
  ask_mode: false,
  mac_titlebar_hidden: false,
};

import Titlebar from '~view/Titlebar';

describe('Titlebar', () => {
  beforeEach(() => {
    mockInvoke.mockReset();
    mockInvoke.mockImplementation((cmd: string) => {
      if (cmd === 'get_app_conf') return Promise.resolve(baseConf);
      return Promise.resolve(undefined);
    });
  });

  it('renders without crashing', async () => {
    await act(async () => {
      render(<Titlebar />);
    });
  });

  it('renders the Reload button', async () => {
    await act(async () => {
      render(<Titlebar />);
    });
    expect(screen.getByText('Reload')).toBeInTheDocument();
  });

  it('calls view_reload when Reload is clicked', async () => {
    await act(async () => {
      render(<Titlebar />);
    });
    await act(async () => {
      fireEvent.click(screen.getByText('Reload'));
    });
    expect(mockInvoke).toHaveBeenCalledWith('view_reload');
  });

  it('calls open_settings when Setting is clicked', async () => {
    await act(async () => {
      render(<Titlebar />);
    });
    await act(async () => {
      fireEvent.click(screen.getByText('Setting'));
    });
    expect(mockInvoke).toHaveBeenCalledWith('open_settings');
  });

  it('calls set_view_ask with enabled=true when Ask is clicked', async () => {
    await act(async () => {
      render(<Titlebar />);
    });
    await act(async () => {
      fireEvent.click(screen.getByText('Ask'));
    });
    expect(mockInvoke).toHaveBeenCalledWith('set_view_ask', { enabled: true });
  });

  it('cycles theme from system → light when ThemeSystem is clicked', async () => {
    await act(async () => {
      render(<Titlebar />);
    });
    await act(async () => {
      fireEvent.click(screen.getByText('ThemeSystem'));
    });
    expect(mockInvoke).toHaveBeenCalledWith('set_theme', { theme: 'light' });
  });

  it('calls window_pin with pin=true when UnPin is clicked', async () => {
    await act(async () => {
      render(<Titlebar />);
    });
    await act(async () => {
      fireEvent.click(screen.getByText('UnPin'));
    });
    expect(mockInvoke).toHaveBeenCalledWith('window_pin', { pin: true });
  });
});
