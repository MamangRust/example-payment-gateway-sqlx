import { debounce } from "@/helpers/debounce";
import useWithdrawStore from "@/store/withdraw/withdraw";
import { useCallback, useMemo } from "react";

export default function useYearAmountWithdraw(toast: any) {
    const {
        findYearWithdrawAmount,
        yearWithdrawAmount,
        loadingYearWithdrawAmount,
        setLoadingYearWithdrawAmount,
        setErrorYearWithdrawAmount,
    } = useWithdrawStore();

    const yearlyAmount = useMemo(() => {
        if (!yearWithdrawAmount || !Array.isArray(yearWithdrawAmount)) {
            return Array(5).fill(0);
        }

        const balanceMap = new Map<number, number>();
        yearWithdrawAmount.forEach((balance) => {
            balanceMap.set(Number(balance.year), balance.total_amount);
        });

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        return years.map((year) => balanceMap.get(year) || 0);
    }, [yearWithdrawAmount]);

    const fetchYearlyAmount = useCallback(
        async (year: number) => {
            try {
                setLoadingYearWithdrawAmount(true);
                setErrorYearWithdrawAmount(null);

                await findYearWithdrawAmount(toast, year);
            } catch (error) {
                setErrorYearWithdrawAmount("Failed to fetch yearly balance");
            } finally {
                setLoadingYearWithdrawAmount(false);
            }
        },
        [findYearWithdrawAmount, setLoadingYearWithdrawAmount, setErrorYearWithdrawAmount],
    );

    const debouncedFetchYearlyAmount = debounce(fetchYearlyAmount, 300);

    return{
        findYearWithdrawAmount,
        yearWithdrawAmount,
        loadingYearWithdrawAmount,
        setLoadingYearWithdrawAmount,
        setErrorYearWithdrawAmount,
        yearlyAmount,
        fetchYearlyAmount,
        debouncedFetchYearlyAmount
    }
}