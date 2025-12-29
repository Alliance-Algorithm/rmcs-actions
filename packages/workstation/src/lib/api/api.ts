export function getEndpoint(path: string): string {
  const baseUrl = 'http://localhost:3000/api';
  return `${baseUrl}${path}`;
}
