export const formatRupiah = (value: number) => {
  const isSupported = Intl.NumberFormat.supportedLocalesOf("id-ID").length > 0;

  if (!isSupported) {
    return `Rp ${value.toLocaleString("en-US", { minimumFractionDigits: 0 })}`;
  }

  return new Intl.NumberFormat("id-ID", {
    style: "currency",
    currency: "IDR",
    minimumFractionDigits: 0,
  }).format(value);
};
