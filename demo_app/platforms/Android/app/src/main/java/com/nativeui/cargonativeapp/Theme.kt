package com.nativeui.cargonativeapp


import android.app.UiModeManager
import android.content.Context
import android.os.Build
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.ColorScheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.material3.dynamicDarkColorScheme
import androidx.compose.material3.dynamicLightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import android.app.Activity
import androidx.compose.runtime.SideEffect
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.platform.LocalView
import androidx.core.view.WindowCompat
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material.icons.rounded.*
import androidx.compose.material.icons.outlined.*
import androidx.compose.material.icons.sharp.*
import androidx.compose.material.icons.twotone.*
import androidx.compose.ui.graphics.vector.ImageVector

private val lightScheme = lightColorScheme (
    primary = Color(0xFF00462E),
    onPrimary = Color(0xFFFFFFFF),
    primaryContainer = Color(0xFF006040),
    onPrimaryContainer = Color(0xFFFFFFFF),
    inversePrimary = Color(0xFF88D7AE),
    secondary = Color(0xFF486555),
    onSecondary = Color(0xFFFFFFFF),
    secondaryContainer = Color(0xFFC7E7D3),
    onSecondaryContainer = Color(0xFF4C6959),
    tertiary = Color(0xFF805600),
    onTertiary = Color(0xFFFFFFFF),
    tertiaryContainer = Color(0xFFDC9B25),
    onTertiaryContainer = Color(0xFF543700),
    background = Color(0xFFF7FAF5),
    onBackground = Color(0xFF181D1A),
    surface = Color(0xFFF7FAF5),
    onSurface = Color(0xFF181D1A),
    surfaceVariant = Color(0xFFDAE5DC),
    onSurfaceVariant = Color(0xFF3F4943),
    inverseSurface = Color(0xFF2D312E),
    inverseOnSurface = Color(0xFFEEF2ED),
    error = Color(0xFFBA1A1A),
    onError = Color(0xFFFFFFFF),
    errorContainer = Color(0xFFFFDAD6),
    onErrorContainer = Color(0xFF93000A),
    outline = Color(0xFF6F7A72),
    outlineVariant = Color(0xFFBEC9C1),
    scrim = Color(0xFF000000),
)

private val darkScheme = darkColorScheme (
    primary = Color(0xFF88D7AE),
    onPrimary = Color(0xFFFFFFFF),
    primaryContainer = Color(0xFF006040),
    onPrimaryContainer = Color(0xFFFFFFFF),
    inversePrimary = Color(0xFF176B4A),
    secondary = Color(0xFFAECEBA),
    onSecondary = Color(0xFF1A3628),
    secondaryContainer = Color(0xFF314D3E),
    onSecondaryContainer = Color(0xFF9DBCA9),
    tertiary = Color(0xFFFFBA45),
    onTertiary = Color(0xFF442C00),
    tertiaryContainer = Color(0xFFDC9B25),
    onTertiaryContainer = Color(0xFF543700),
    background = Color(0xFF101412),
    onBackground = Color(0xFFE0E3DE),
    surface = Color(0xFF101412),
    onSurface = Color(0xFFE0E3DE),
    surfaceVariant = Color(0xFF3F4943),
    onSurfaceVariant = Color(0xFFBEC9C1),
    inverseSurface = Color(0xFFE0E3DE),
    inverseOnSurface = Color(0xFF2D312E),
    error = Color(0xFFFFB4AB),
    onError = Color(0xFF690005),
    errorContainer = Color(0xFF93000A),
    onErrorContainer = Color(0xFFFFDAD6),
    outline = Color(0xFF89938B),
    outlineVariant = Color(0xFF3F4943),
    scrim = Color(0xFF000000),
)

private val mediumContrastLightColorScheme = lightColorScheme (
    primary = Color(0xFF003F28),
    onPrimary = Color(0xFFFFFFFF),
    primaryContainer = Color(0xFF006040),
    onPrimaryContainer = Color(0xFFFFFFFF),
    inversePrimary = Color(0xFF88D7AE),
    secondary = Color(0xFF203C2E),
    onSecondary = Color(0xFFFFFFFF),
    secondaryContainer = Color(0xFF567363),
    onSecondaryContainer = Color(0xFFFFFFFF),
    tertiary = Color(0xFF4B3100),
    onTertiary = Color(0xFFFFFFFF),
    tertiaryContainer = Color(0xFF936400),
    onTertiaryContainer = Color(0xFFFFFFFF),
    background = Color(0xFFF7FAF5),
    onBackground = Color(0xFF181D1A),
    surface = Color(0xFFF7FAF5),
    onSurface = Color(0xFF0E120F),
    surfaceVariant = Color(0xFFDAE5DC),
    onSurfaceVariant = Color(0xFF2F3832),
    inverseSurface = Color(0xFF2D312E),
    inverseOnSurface = Color(0xFFEEF2ED),
    error = Color(0xFF740006),
    onError = Color(0xFFFFFFFF),
    errorContainer = Color(0xFFCF2C27),
    onErrorContainer = Color(0xFFFFFFFF),
    outline = Color(0xFF4B554E),
    outlineVariant = Color(0xFF657068),
    scrim = Color(0xFF000000),
)

private val highContrastLightColorScheme = lightColorScheme (
    primary = Color(0xFF003420),
    onPrimary = Color(0xFFFFFFFF),
    primaryContainer = Color(0xFF005438),
    onPrimaryContainer = Color(0xFFFFFFFF),
    inversePrimary = Color(0xFF88D7AE),
    secondary = Color(0xFF163124),
    onSecondary = Color(0xFFFFFFFF),
    secondaryContainer = Color(0xFF334F40),
    onSecondaryContainer = Color(0xFFFFFFFF),
    tertiary = Color(0xFF3E2800),
    onTertiary = Color(0xFFFFFFFF),
    tertiaryContainer = Color(0xFF644200),
    onTertiaryContainer = Color(0xFFFFFFFF),
    background = Color(0xFFF7FAF5),
    onBackground = Color(0xFF181D1A),
    surface = Color(0xFFF7FAF5),
    onSurface = Color(0xFF000000),
    surfaceVariant = Color(0xFFDAE5DC),
    onSurfaceVariant = Color(0xFF000000),
    inverseSurface = Color(0xFF2D312E),
    inverseOnSurface = Color(0xFFFFFFFF),
    error = Color(0xFF600004),
    onError = Color(0xFFFFFFFF),
    errorContainer = Color(0xFF98000A),
    onErrorContainer = Color(0xFFFFFFFF),
    outline = Color(0xFF252E28),
    outlineVariant = Color(0xFF414B45),
    scrim = Color(0xFF000000),
)

private val mediumContrastDarkColorScheme = darkColorScheme (
    primary = Color(0xFF9EEDC3),
    onPrimary = Color(0xFFFFFFFF),
    primaryContainer = Color(0xFF529F7A),
    onPrimaryContainer = Color(0xFF000000),
    inversePrimary = Color(0xFF005337),
    secondary = Color(0xFFC4E4D0),
    onSecondary = Color(0xFF0F2B1E),
    secondaryContainer = Color(0xFF799886),
    onSecondaryContainer = Color(0xFF000000),
    tertiary = Color(0xFFFFD69C),
    onTertiary = Color(0xFF362200),
    tertiaryContainer = Color(0xFFDC9B25),
    onTertiaryContainer = Color(0xFF2A1900),
    background = Color(0xFF101412),
    onBackground = Color(0xFFE0E3DE),
    surface = Color(0xFF101412),
    onSurface = Color(0xFFFFFFFF),
    surfaceVariant = Color(0xFF3F4943),
    onSurfaceVariant = Color(0xFFD4DFD6),
    inverseSurface = Color(0xFFE0E3DE),
    inverseOnSurface = Color(0xFF272B28),
    error = Color(0xFFFFD2CC),
    onError = Color(0xFF540003),
    errorContainer = Color(0xFFFF5449),
    onErrorContainer = Color(0xFF000000),
    outline = Color(0xFFAAB5AC),
    outlineVariant = Color(0xFF88938B),
    scrim = Color(0xFF000000),
)

private val highContrastDarkColorScheme = darkColorScheme (
    primary = Color(0xFFBAFFD9),
    onPrimary = Color(0xFFFFFFFF),
    primaryContainer = Color(0xFF84D3AA),
    onPrimaryContainer = Color(0xFF000E07),
    inversePrimary = Color(0xFF005337),
    secondary = Color(0xFFD7F8E3),
    onSecondary = Color(0xFF000000),
    secondaryContainer = Color(0xFFABCAB7),
    onSecondaryContainer = Color(0xFF000E07),
    tertiary = Color(0xFFFFEDD9),
    onTertiary = Color(0xFF000000),
    tertiaryContainer = Color(0xFFFBB640),
    onTertiaryContainer = Color(0xFF130900),
    background = Color(0xFF101412),
    onBackground = Color(0xFFE0E3DE),
    surface = Color(0xFF101412),
    onSurface = Color(0xFFFFFFFF),
    surfaceVariant = Color(0xFF3F4943),
    onSurfaceVariant = Color(0xFFFFFFFF),
    inverseSurface = Color(0xFFE0E3DE),
    inverseOnSurface = Color(0xFF000000),
    error = Color(0xFFFFECE9),
    onError = Color(0xFF000000),
    errorContainer = Color(0xFFFFAEA4),
    onErrorContainer = Color(0xFF220001),
    outline = Color(0xFFE8F3EA),
    outlineVariant = Color(0xFFBBC5BD),
    scrim = Color(0xFF000000),
)

@Composable
fun isHighContrastEnabled(): Boolean {
    val context = LocalContext.current
    return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
        val uiModeManager = context.getSystemService(Context.UI_MODE_SERVICE) as? UiModeManager
        (uiModeManager?.contrast ?: 0f) >= 0.66f
    } else {
        false
    }
}

@Composable
fun isMediumContrastEnabled(): Boolean {
    val context = LocalContext.current
    return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
        val uiModeManager = context.getSystemService(Context.UI_MODE_SERVICE) as? UiModeManager
        val c = uiModeManager?.contrast ?: 0f
        c >= 0.33f && c < 0.66f
    } else {
        false
    }
}

@Composable
fun AppTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    dynamicColor: Boolean = false,
    content: @Composable () -> Unit
) {
    val isHighContrast = isHighContrastEnabled()
    val isMediumContrast = isMediumContrastEnabled()

    val colorScheme = when {
        dynamicColor && Build.VERSION.SDK_INT >= Build.VERSION_CODES.S -> {
            val context = LocalContext.current
            if (darkTheme) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
        }
        darkTheme && isHighContrast -> highContrastDarkColorScheme
        darkTheme && isMediumContrast -> mediumContrastDarkColorScheme
        darkTheme -> darkScheme
        isHighContrast -> highContrastLightColorScheme
        isMediumContrast -> mediumContrastLightColorScheme
        else -> lightScheme
    }

    val view = LocalView.current
    if (!view.isInEditMode) {
        SideEffect {
            val window = (view.context as Activity).window
            WindowCompat.getInsetsController(window, view).isAppearanceLightStatusBars = !darkTheme
            WindowCompat.getInsetsController(window, view).isAppearanceLightNavigationBars = !darkTheme
        }
    }

    MaterialTheme(
        colorScheme = colorScheme,
        content = content
    )
}

const val AppIconStyle = "rounded"

val AppBackIcon: ImageVector = when (AppIconStyle) {
    "rounded" -> Icons.Rounded.ArrowBack
    "outlined" -> Icons.Outlined.ArrowBack
    "sharp" -> Icons.Sharp.ArrowBack
    "twotone", "two-tone" -> Icons.TwoTone.ArrowBack
    else -> Icons.Filled.ArrowBack
}

val AppCloseIcon: ImageVector = when (AppIconStyle) {
    "rounded" -> Icons.Rounded.Close
    "outlined" -> Icons.Outlined.Close
    "sharp" -> Icons.Sharp.Close
    "twotone", "two-tone" -> Icons.TwoTone.Close
    else -> Icons.Filled.Close
}

fun appTabIcon(iconName: String): ImageVector = when (iconName.lowercase()) {
    "arrow_left" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.ArrowBack
        "outlined" -> Icons.Outlined.ArrowBack
        "sharp" -> Icons.Sharp.ArrowBack
        "twotone", "two-tone" -> Icons.TwoTone.ArrowBack
        else -> Icons.Filled.ArrowBack
    }
    "arrow_right" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.ArrowForward
        "outlined" -> Icons.Outlined.ArrowForward
        "sharp" -> Icons.Sharp.ArrowForward
        "twotone", "two-tone" -> Icons.TwoTone.ArrowForward
        else -> Icons.Filled.ArrowForward
    }
    "bell" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Notifications
        "outlined" -> Icons.Outlined.Notifications
        "sharp" -> Icons.Sharp.Notifications
        "twotone", "two-tone" -> Icons.TwoTone.Notifications
        else -> Icons.Filled.Notifications
    }
    "bolt" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Build
        "outlined" -> Icons.Outlined.Build
        "sharp" -> Icons.Sharp.Build
        "twotone", "two-tone" -> Icons.TwoTone.Build
        else -> Icons.Filled.Build
    }
    "cart", "bag" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.ShoppingCart
        "outlined" -> Icons.Outlined.ShoppingCart
        "sharp" -> Icons.Sharp.ShoppingCart
        "twotone", "two-tone" -> Icons.TwoTone.ShoppingCart
        else -> Icons.Filled.ShoppingCart
    }
    "check" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Check
        "outlined" -> Icons.Outlined.Check
        "sharp" -> Icons.Sharp.Check
        "twotone", "two-tone" -> Icons.TwoTone.Check
        else -> Icons.Filled.Check
    }
    "check_circle" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.CheckCircle
        "outlined" -> Icons.Outlined.CheckCircle
        "sharp" -> Icons.Sharp.CheckCircle
        "twotone", "two-tone" -> Icons.TwoTone.CheckCircle
        else -> Icons.Filled.CheckCircle
    }
    "chevron_left" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.KeyboardArrowLeft
        "outlined" -> Icons.Outlined.KeyboardArrowLeft
        "sharp" -> Icons.Sharp.KeyboardArrowLeft
        "twotone", "two-tone" -> Icons.TwoTone.KeyboardArrowLeft
        else -> Icons.Filled.KeyboardArrowLeft
    }
    "chevron_right" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.KeyboardArrowRight
        "outlined" -> Icons.Outlined.KeyboardArrowRight
        "sharp" -> Icons.Sharp.KeyboardArrowRight
        "twotone", "two-tone" -> Icons.TwoTone.KeyboardArrowRight
        else -> Icons.Filled.KeyboardArrowRight
    }
    "close" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Close
        "outlined" -> Icons.Outlined.Close
        "sharp" -> Icons.Sharp.Close
        "twotone", "two-tone" -> Icons.TwoTone.Close
        else -> Icons.Filled.Close
    }
    "coffee", "mug" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Coffee
        "outlined" -> Icons.Outlined.Coffee
        "sharp" -> Icons.Sharp.Coffee
        "twotone", "two-tone" -> Icons.TwoTone.Coffee
        else -> Icons.Filled.Coffee
    }
    "edit" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Edit
        "outlined" -> Icons.Outlined.Edit
        "sharp" -> Icons.Sharp.Edit
        "twotone", "two-tone" -> Icons.TwoTone.Edit
        else -> Icons.Filled.Edit
    }
    "heart" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Favorite
        "outlined" -> Icons.Outlined.Favorite
        "sharp" -> Icons.Sharp.Favorite
        "twotone", "two-tone" -> Icons.TwoTone.Favorite
        else -> Icons.Filled.Favorite
    }
    "home", "house" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Home
        "outlined" -> Icons.Outlined.Home
        "sharp" -> Icons.Sharp.Home
        "twotone", "two-tone" -> Icons.TwoTone.Home
        else -> Icons.Filled.Home
    }
    "info" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Info
        "outlined" -> Icons.Outlined.Info
        "sharp" -> Icons.Sharp.Info
        "twotone", "two-tone" -> Icons.TwoTone.Info
        else -> Icons.Filled.Info
    }
    "lock" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Lock
        "outlined" -> Icons.Outlined.Lock
        "sharp" -> Icons.Sharp.Lock
        "twotone", "two-tone" -> Icons.TwoTone.Lock
        else -> Icons.Filled.Lock
    }
    "mail" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Email
        "outlined" -> Icons.Outlined.Email
        "sharp" -> Icons.Sharp.Email
        "twotone", "two-tone" -> Icons.TwoTone.Email
        else -> Icons.Filled.Email
    }
    "navigate_back" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.ArrowBack
        "outlined" -> Icons.Outlined.ArrowBack
        "sharp" -> Icons.Sharp.ArrowBack
        "twotone", "two-tone" -> Icons.TwoTone.ArrowBack
        else -> Icons.Filled.ArrowBack
    }
    "orders", "order", "bag", "cart" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.ShoppingCart
        "outlined" -> Icons.Outlined.ShoppingCart
        "sharp" -> Icons.Sharp.ShoppingCart
        "twotone", "two-tone" -> Icons.TwoTone.ShoppingCart
        else -> Icons.Filled.ShoppingCart
    }
    "plus" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Add
        "outlined" -> Icons.Outlined.Add
        "sharp" -> Icons.Sharp.Add
        "twotone", "two-tone" -> Icons.TwoTone.Add
        else -> Icons.Filled.Add
    }
    "qrcode", "qr_code", "scan" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.QrCode
        "outlined" -> Icons.Outlined.QrCode
        "sharp" -> Icons.Sharp.QrCode
        "twotone", "two-tone" -> Icons.TwoTone.QrCode
        else -> Icons.Filled.QrCode
    }
    "refresh" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Refresh
        "outlined" -> Icons.Outlined.Refresh
        "sharp" -> Icons.Sharp.Refresh
        "twotone", "two-tone" -> Icons.TwoTone.Refresh
        else -> Icons.Filled.Refresh
    }
    "search" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Search
        "outlined" -> Icons.Outlined.Search
        "sharp" -> Icons.Sharp.Search
        "twotone", "two-tone" -> Icons.TwoTone.Search
        else -> Icons.Filled.Search
    }
    "settings", "gear" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Settings
        "outlined" -> Icons.Outlined.Settings
        "sharp" -> Icons.Sharp.Settings
        "twotone", "two-tone" -> Icons.TwoTone.Settings
        else -> Icons.Filled.Settings
    }
    "share" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Share
        "outlined" -> Icons.Outlined.Share
        "sharp" -> Icons.Sharp.Share
        "twotone", "two-tone" -> Icons.TwoTone.Share
        else -> Icons.Filled.Share
    }
    "sparkles" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Star
        "outlined" -> Icons.Outlined.Star
        "sharp" -> Icons.Sharp.Star
        "twotone", "two-tone" -> Icons.TwoTone.Star
        else -> Icons.Filled.Star
    }
    "star" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Star
        "outlined" -> Icons.Outlined.Star
        "sharp" -> Icons.Sharp.Star
        "twotone", "two-tone" -> Icons.TwoTone.Star
        else -> Icons.Filled.Star
    }
    "trash" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Delete
        "outlined" -> Icons.Outlined.Delete
        "sharp" -> Icons.Sharp.Delete
        "twotone", "two-tone" -> Icons.TwoTone.Delete
        else -> Icons.Filled.Delete
    }
    "user", "person", "profile" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Person
        "outlined" -> Icons.Outlined.Person
        "sharp" -> Icons.Sharp.Person
        "twotone", "two-tone" -> Icons.TwoTone.Person
        else -> Icons.Filled.Person
    }
    "warning" -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Warning
        "outlined" -> Icons.Outlined.Warning
        "sharp" -> Icons.Sharp.Warning
        "twotone", "two-tone" -> Icons.TwoTone.Warning
        else -> Icons.Filled.Warning
    }
    else -> when (AppIconStyle) {
        "rounded" -> Icons.Rounded.Star
        "outlined" -> Icons.Outlined.Star
        "sharp" -> Icons.Sharp.Star
        "twotone", "two-tone" -> Icons.TwoTone.Star
        else -> Icons.Filled.Star
    }
}
