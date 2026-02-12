export function time_to_wait(time: string): number {
  const [scheduledHour, scheduledMinute] = time.split(':').map((component) => parseInt(component));
  const scheduledTime = (scheduledMinute * 60 + scheduledHour * 3600) * 1000;
  const d = new Date();
  const currentHour = d.getHours();
  const currentMinute = d.getMinutes();
  const currentTime = (currentMinute * 60 + currentHour * 3600) * 1000;
  if (scheduledTime > currentTime) {
    return scheduledTime - currentTime;
  } else {
    return 24 * 3600 * 1000 - currentTime + scheduledTime;
  }
}
export function time_to_ms(time: string): number {
  const [h, m] = time.split(':').map((component) => parseInt(component));
  return (m * 60 + h * 3600) * 1000;
}
