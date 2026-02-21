type Listener<T = any> = (data: T) => void;
type PathValue = any;

export class Storage {
  private data: Record<string, any> = {};
  private listeners: Map<string, Set<Listener>> = new Map();

  constructor(initialData?: Record<string, any>) {
    if (initialData) {
      this.data = JSON.parse(JSON.stringify(initialData));
    }
  }

  /**
   * Set a value at the given path
   * @param path - Dot-separated path (e.g., "user.name" or "game.settings")
   * @param value - The value to set
   */
  set(path: string, value: PathValue): void {
    const oldValue = this.get(path);
    this.setByPath(this.data, path, value);

    // Notify listeners for this exact path
    this.notifyListeners(path, value);

    // Notify listeners for parent paths
    const pathParts = path.split(".");
    for (let i = pathParts.length - 1; i > 0; i--) {
      const parentPath = pathParts.slice(0, i).join(".");
      const parentValue = this.get(parentPath);
      this.notifyListeners(parentPath, parentValue);
    }

    // Notify wildcard listeners
    this.notifyListeners("*", { path, value, oldValue });
  }

  /**
   * Get a value at the given path
   * @param path - Dot-separated path
   * @returns The value at the path, or undefined if not found
   */
  get<T = any>(path: string): T | undefined {
    return this.getByPath(this.data, path);
  }

  /**
   * Check if a path exists
   * @param path - Dot-separated path
   * @returns true if the path exists
   */
  has(path: string): boolean {
    return this.get(path) !== undefined;
  }

  /**
   * Delete a value at the given path
   * @param path - Dot-separated path
   */
  delete(path: string): void {
    const oldValue = this.get(path);
    this.deleteByPath(this.data, path);

    // Notify listeners
    this.notifyListeners(path, undefined);

    // Notify parent paths
    const pathParts = path.split(".");
    for (let i = pathParts.length - 1; i > 0; i--) {
      const parentPath = pathParts.slice(0, i).join(".");
      const parentValue = this.get(parentPath);
      this.notifyListeners(parentPath, parentValue);
    }

    // Notify wildcard listeners
    this.notifyListeners("*", { path, value: undefined, oldValue });
  }

  /**
   * Register a listener for changes at the given path
   * @param path - Dot-separated path or "*" for all changes
   * @param callback - Function to call when the value changes
   * @returns Unsubscribe function
   */
  onChange<T = any>(path: string, callback: Listener<T>): () => void {
    if (!this.listeners.has(path)) {
      this.listeners.set(path, new Set());
    }
    this.listeners.get(path)!.add(callback);

    // Return unsubscribe function
    return () => {
      const listeners = this.listeners.get(path);
      if (listeners) {
        listeners.delete(callback);
        if (listeners.size === 0) {
          this.listeners.delete(path);
        }
      }
    };
  }

  /**
   * Remove a specific listener
   * @param path - Dot-separated path
   * @param callback - The callback to remove
   */
  off<T = any>(path: string, callback: Listener<T>): void {
    const listeners = this.listeners.get(path);
    if (listeners) {
      listeners.delete(callback);
      if (listeners.size === 0) {
        this.listeners.delete(path);
      }
    }
  }

  /**
   * Remove all listeners for a path
   * @param path - Dot-separated path
   */
  clearListeners(path: string): void {
    this.listeners.delete(path);
  }

  /**
   * Remove all listeners
   */
  clearAllListeners(): void {
    this.listeners.clear();
  }

  /**
   * Get all data
   */
  getAll(): Record<string, any> {
    return JSON.parse(JSON.stringify(this.data));
  }

  /**
   * Clear all data
   */
  clear(): void {
    const oldData = this.getAll();
    this.data = {};

    // Notify all listeners
    for (const [path] of this.listeners) {
      if (path !== "*") {
        this.notifyListeners(path, undefined);
      }
    }

    this.notifyListeners("*", {
      path: "",
      value: undefined,
      oldValue: oldData,
    });
  }

  private notifyListeners(path: string, value: any): void {
    const listeners = this.listeners.get(path);
    if (listeners) {
      listeners.forEach((callback) => {
        try {
          callback(value);
        } catch (error) {
          console.error(`Error in listener for path "${path}":`, error);
        }
      });
    }
  }

  private setByPath(obj: any, path: string, value: any): void {
    const parts = path.split(".");
    let current = obj;

    for (let i = 0; i < parts.length - 1; i++) {
      const part = parts[i];
      if (!(part in current) || typeof current[part] !== "object") {
        current[part] = {};
      }
      current = current[part];
    }

    current[parts[parts.length - 1]] = value;
  }

  private getByPath(obj: any, path: string): any {
    const parts = path.split(".");
    let current = obj;

    for (const part of parts) {
      if (current == null || typeof current !== "object") {
        return undefined;
      }
      current = current[part];
    }

    return current;
  }

  private deleteByPath(obj: any, path: string): void {
    const parts = path.split(".");
    let current = obj;

    for (let i = 0; i < parts.length - 1; i++) {
      const part = parts[i];
      if (!(part in current) || typeof current[part] !== "object") {
        return;
      }
      current = current[part];
    }

    delete current[parts[parts.length - 1]];
  }
}

// Create and export a singleton instance
export const storage = new Storage();

// Export for creating multiple instances if needed
export default Storage;
