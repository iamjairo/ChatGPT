import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';

const { mockPlatform } = vi.hoisted(() => ({ mockPlatform: vi.fn() }));

vi.mock('@tauri-apps/plugin-os', () => ({
  platform: mockPlatform,
}));

import useInfo from '~hooks/useInfo';

describe('useInfo', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('initializes with empty platform and isMac=false', () => {
    mockPlatform.mockResolvedValue('linux');
    const { result } = renderHook(() => useInfo());
    expect(result.current.platform).toBe('');
    expect(result.current.isMac).toBe(false);
  });

  it('sets platform to "linux" when OS returns linux', async () => {
    mockPlatform.mockResolvedValue('linux');
    const { result } = renderHook(() => useInfo());
    await act(async () => {
      await Promise.resolve();
    });
    expect(result.current.platform).toBe('linux');
    expect(result.current.isMac).toBe(false);
  });

  it('sets isMac=true when OS returns macos', async () => {
    mockPlatform.mockResolvedValue('macos');
    const { result } = renderHook(() => useInfo());
    await act(async () => {
      await Promise.resolve();
    });
    expect(result.current.isMac).toBe(true);
    expect(result.current.platform).toBe('macos');
  });

  it('sets platform to "windows" and isMac=false for windows', async () => {
    mockPlatform.mockResolvedValue('windows');
    const { result } = renderHook(() => useInfo());
    await act(async () => {
      await Promise.resolve();
    });
    expect(result.current.platform).toBe('windows');
    expect(result.current.isMac).toBe(false);
  });
});
