export const toNormalFormat = (date: Date) => {
  const year = date.getFullYear();
  const month = date.getMonth();
  const day = date.getDate();
  const hours = date.getHours();
  const minutes = date.getMinutes();

  return `${year}/${String(month + 1).padStart(2, '0')}/${String(day).padStart(2, '0')} ${hours}:${minutes}`;
};