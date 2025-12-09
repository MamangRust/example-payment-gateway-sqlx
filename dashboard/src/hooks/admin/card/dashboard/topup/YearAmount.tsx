import { debounce } from "@/helpers/debounce";
import useTopupStore from "@/store/topup/topup";
import { useCallback, useMemo } from "react";

export default function useYearAmountTopup(toast: any) {
    const {
        findYearTopupAmount,
        yearTopupAmount,
        loadingYearTopupAmount,
        setLoadingYearTopupAmount,
        setErrorYearTopupAmount,
    } = useTopupStore();

    const yearlyAmount = useMemo(() => {
        if (!yearTopupAmount || !Array.isArray(yearTopupAmount)) {
            return Array(5).fill(0);
        }

        const balanceMap = new Map<number, number>();
        yearTopupAmount.forEach((balance) => {
            balanceMap.set(Number(balance.year), balance.total_amount);
        });

        const currentYear = new Date().getFullYear();
        const years = Array.from({ length: 5 }, (_, i) => currentYear - 4 + i);

        return years.map((year) => balanceMap.get(year) || 0);
    }, [yearTopupAmount]);

    const fetchYearlyAmount = useCallback(
        async (year: number) => {
            try {
                setLoadingYearTopupAmount(true);
                setErrorYearTopupAmount(null);

                await findYearTopupAmount(toast, year);
            } catch (error) {
                setErrorYearTopupAmount("Failed to fetch yearly balance");
            } finally {
                setLoadingYearTopupAmount(false);
            }
        },
        [findYearTopupAmount, setLoadingYearTopupAmount, setErrorYearTopupAmount],
    );

    const debouncedFetchYearlyAmount = debounce(fetchYearlyAmount, 300);

    return{
        findYearTopupAmount,
        yearTopupAmount,
        loadingYearTopupAmount,
        setLoadingYearTopupAmount,
        setErrorYearTopupAmount,
        yearlyAmount,
        fetchYearlyAmount,
        debouncedFetchYearlyAmount
    }
}