import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';

const { mockTheme, mockOnThemeChanged, mockUnlisten } = vi.hoisted(() => ({
  mockTheme: vi.fn().mockResolvedValue('light'),
  mockOnThemeChanged: vi.fn(),
  mockUnlisten: vi.fn(),
}));

vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: vi.fn(() => ({
    theme: mockTheme,
    onThemeChanged: mockOnThemeChanged,
  })),
}));

import useTheme from '~hooks/useTheme';

describe('useTheme', () => {
  let themeChangedCallback: ((event: { payload: string }) => void) | null = null;

  beforeEach(() => {
    vi.clearAllMocks();
    themeChangedCallback = null;
    mockTheme.mockResolvedValue('light');
    mockOnThemeChanged.mockImplementation(async (cb: (e: { payload: string }) => void) => {
      themeChangedCallback = cb;
      return mockUnlisten;
    });
  });

  it('initializes with "light" as default theme', () => {
    const { result } = renderHook(() => useTheme());
    expect(result.current).toBe('light');
  });

  it('sets theme from window.theme() on mount', async () => {
    mockTheme.mockResolvedValue('dark');
    const { result } = renderHook(() => useTheme());
    await act(async () => {
      await Promise.resolve();
    });
    expect(result.current).toBe('dark');
  });

  it('updates theme when onThemeChanged fires', async () => {
    const { result } = renderHook(() => useTheme());
    await act(async () => {
      await Promise.resolve();
    });
    await act(async () => {
      themeChangedCallback?.({ payload: 'dark' });
    });
    expect(result.current).toBe('dark');
  });

  it('falls back to empty string when window.theme() returns null', async () => {
    mockTheme.mockResolvedValue(null);
    const { result } = renderHook(() => useTheme());
    await act(async () => {
      await Promise.resolve();
    });
    expect(result.current).toBe('');
  });

  it('calls unlisten on unmount', async () => {
    const { unmount } = renderHook(() => useTheme());
    await act(async () => {
      await Promise.resolve();
    });
    unmount();
    expect(mockUnlisten).toHaveBeenCalled();
  });
});
