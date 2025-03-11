import { browser } from '$app/environment';
import { init, register } from 'svelte-i18n';

register('pl', () => import('../locales/pl.json'));
register('en', () => import('../locales/en.json'));

const defaultLocale = 'en';

init({
	fallbackLocale: defaultLocale,
	initialLocale: browser ? window.navigator.language : defaultLocale
});
