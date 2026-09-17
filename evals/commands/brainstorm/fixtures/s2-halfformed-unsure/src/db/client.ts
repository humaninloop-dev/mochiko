// Thin Postgres client wrapper used across src/. Real implementation lives in the
// deployed service; this is the interface the schedule modules code against.

export type Tx = {
  query<T = unknown>(sql: string, params?: unknown[]): Promise<T[]>;
  one<T = unknown>(sql: string, params?: unknown[]): Promise<T>;
};

export type Db = Tx & {
  tx<T>(fn: (t: Tx) => Promise<T>): Promise<T>;
};
