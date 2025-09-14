import { calculatePercentageChange } from "@/helpers/calculate";
import { debounce } from "@/helpers/debounce";
import { monthNames } from "@/helpers/month";
import useMerchantStore from "@/store/merchant/merchant";
import { useCallback } from "react";


export default function useMonthTotalAmountMerchant(currentMonths: number, currentYear: number,previousMonth: number, previousYear: number, toast: any) {
    const {
        findMonthTotalAmount,
        monthTotalAmount,
        loadingMonthTotalAmount,
        setLoadingMonthTotalAmount,
        setErrorMonthTotalAmount,
    } = useMerchantStore();

    const currentMonthData =
        monthTotalAmount && Array.isArray(monthTotalAmount)
            ? monthTotalAmount.find(
                (balance) =>
                    balance.month === monthNames[currentMonths] &&
                    balance.year === currentYear.toString(),
            )
            : null;

    const previousMonthData =
        monthTotalAmount && Array.isArray(monthTotalAmount)
            ? monthTotalAmount.find(
                (balance) =>
                    balance.month === monthNames[previousMonth] &&
                    balance.year === previousYear.toString(),
            )
            : null;

    const currentMonthBalance = currentMonthData?.total_amount || 0;
    const previousMonthBalance = previousMonthData?.total_amount || 0;

    const monthPercentageChange = calculatePercentageChange(
        currentMonthBalance,
        previousMonthBalance,
    );

    const fetchMonthlyTotalbalance = useCallback(
        async (year: number, month: number) => {
            try {
                setLoadingMonthTotalAmount(true);
                setErrorMonthTotalAmount(null);

                await findMonthTotalAmount(toast, year, month);
            } catch (error) {
                setErrorMonthTotalAmount("Failed to fetch monthly balance");
            } finally {
                setLoadingMonthTotalAmount(false);
            }
        },
        [
            findMonthTotalAmount,
            setLoadingMonthTotalAmount,
            setErrorMonthTotalAmount,
        ],
    );

    const debouncedFetchMonthlyTotalBalance = debounce(
        fetchMonthlyTotalbalance,
        1000,
      );

    return {
        currentMonthData,
        previousMonthData,
        currentMonthBalance,
        debouncedFetchMonthlyTotalBalance,
        findMonthTotalAmount,
        monthTotalAmount,
        loadingMonthTotalAmount,
        setLoadingMonthTotalAmount,
        setErrorMonthTotalAmount,
        fetchMonthlyTotalbalance,
        monthPercentageChange
    }
}