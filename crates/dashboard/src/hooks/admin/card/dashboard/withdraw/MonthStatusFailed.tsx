import { calculatePercentageChange } from "@/helpers/calculate";
import { debounce } from "@/helpers/debounce";
import { monthNames } from "@/helpers/month";
import useWithdrawStore from "@/store/withdraw/withdraw";
import { useCallback } from "react";


export default function useMonthStatusFailedWithdraw(currentMonths: number, currentYear: number, previousMonth: number, previousYear: number, toast: any) {
    const {
        findMonthStatusFailed,
        monthStatusFailed,
        loadingMonthStatusFailed,
        setLoadingMonthStatusFailed,
        setErrorMonthStatusFailed,
    } = useWithdrawStore();

    const currentMonthFailedData = monthStatusFailed?.find(
        (balance) =>
            balance.month === monthNames[currentMonths] &&
            balance.year === currentYear.toString(),
    );

    const previousMonthFailedData = monthStatusFailed?.find(
        (balance) =>
            balance.month === monthNames[previousMonth] &&
            balance.year === previousYear.toString(),
    );

    const currentMonthFailed = currentMonthFailedData?.total_amount || 0;
    const previousMonthFailed = previousMonthFailedData?.total_amount || 0;

    const monthPercentageFailedChange = calculatePercentageChange(
        currentMonthFailed,
        previousMonthFailed,
    );


    const fetchMonthlyFailed = useCallback(
        async (year: number, month: number) => {
            try {
                setLoadingMonthStatusFailed(true);
                setErrorMonthStatusFailed(null);

                await findMonthStatusFailed(toast, year, month);
            } catch (error) {
                setErrorMonthStatusFailed("Failed to fetch monthly balance");
            } finally {
                setLoadingMonthStatusFailed(false);
            }
        },
        [
            findMonthStatusFailed,
            setLoadingMonthStatusFailed,
            setErrorMonthStatusFailed,
        ],
    );

    const debouncedFetchMonthlyFailed = debounce(fetchMonthlyFailed, 300);

    return{
        monthPercentageFailedChange,
        loadingMonthStatusFailed,
        currentMonthFailed,
        previousMonthFailed,
        debouncedFetchMonthlyFailed
    }
}